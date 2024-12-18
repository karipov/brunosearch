
import json, os, html

from scrape_courses import get_course_metadata, parallel_get_course_details, clean_description
from scrape_departments import fetch_and_parse_subjects


def structure_course_metadata(courses: list, details: dict, subjects: dict) -> dict:
    """
    Structures course data correctly, including filtering, extracting attributes, combining
    metadata from multiple sources, etc.
    """
    print("[INFO] Structuring course metadata...")
    classes = []
    seen_courses = set()  # track unique (department_short, code) pairs
    
    for course in courses:
        course_detail = details[course["code"]]
        
        # skip courses that are: online, taught by a team, cross-listed
        if (course["meets"] == "Course offered online")\
           or (course["instr"] == "Team")\
           or (course["stat"] == "X"):
               continue

        # e.g. split "CSCI 0320" into "CSCI" and "0320"
        department_short, code = course["code"].split(" ")
        
        # check if we've already processed this course
        if (department_short, code) in seen_courses: continue
        seen_courses.add((department_short, code))
        
        course_datum = {
            "department_full": subjects.get(department_short, department_short),
            "department_short": department_short,
            "code": code,
            "title": course["title"],
            "professor": course["instr"],
            "time": course["meets"],
            "description": clean_description(course_detail["description"]),
            "writ": "WRIT" in course_detail["attr_html"]\
                            or department_short in ["ENGL", "COLT", "LITA", "LITR"],
            "fys": "FYS" in course_detail["attr_html"],
            "soph": "SOPH" in course_detail["attr_html"],
            "rpp": "RPP" in course_detail["attr_html"],
        }
        attributes."writ" if course_datum["writ"] else ""\
                                   + "fys" if course_datum["fys"] else ""\
                                   + "soph" if course_datum["soph"] else ""\
                                   + "rpp" if course_datum["rpp"] else ""

        course_datum["attributes"]

        classes.append(course_datum)
    
    return classes


def scrape(season: str, year: str, output_file: str):
    """
    Scrapes CAB + other sources for current course data and writes it to a JSON file.
    """
    # get all course metadata, all course details and subjects
    courses = get_course_metadata(season, year)
    details = parallel_get_course_details(courses)
    subjects = fetch_and_parse_subjects()
    
    # structure the course metadata correctly
    structured_courses = structure_course_metadata(courses, details, subjects)

    # write classes to a JSON file
    print(f"[INFO] Writing to {len(structured_courses)} processed courses to file...")
    classes_json = json.dumps(structured_courses, ensure_ascii=False, indent=4, sort_keys=True)
    os.makedirs(os.path.dirname(output_file), exist_ok=True)
    with open(output_file, "w", encoding="utf-8") as class_list_file:
        class_list_file.write(classes_json)

    print("[INFO] Done!")


if __name__ == "__main__":
    SEASON = "spring"
    YEAR = "2025"
    
    scrape(SEASON, YEAR, f"../data/{SEASON + YEAR}/courses.json")