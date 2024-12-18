import grequests
import requests
import warnings
from bs4 import BeautifulSoup, MarkupResemblesLocatorWarning
warnings.filterwarnings("ignore", category=MarkupResemblesLocatorWarning)

CAB = "https://cab.brown.edu"
CAB_COURSE_SEARCH_URL = CAB + "/api/?page=fose&route=search&is_ind_study=N&is_canc=N"
CAB_DETAIL_SEARCH_URL = CAB + "/api/?page=fose&route=details"


def construct_db_string(season: str, year: str):
    """
    Converts a school year and season to the corresponding CAB database string, e.g.
        spring 2023 => 202220
        fall 2022 => 202210
    """
    database_year = int(year) if season == "fall" else int(year) - 1
    suffix = "20" if season == "spring" else "10"

    return str(database_year) + suffix


def get_course_metadata(season: str, year: str) -> list:
    """
    Fetches course metadata from CAB for a given term
    """
    print(f"[INFO] Requesting course metadata for {season}, {year}...")
    cab_search_payload = {
        "other": { "srcdb": construct_db_string(season, year)},
        "criteria": [
            { "field": "is_ind_study", "value": "N" },
            { "field": "is_canc", "value": "N" },
        ],
    }
    return requests.post(CAB_COURSE_SEARCH_URL, json=cab_search_payload).json()["results"]


def parallel_get_course_details(courses: list) -> dict:
    """
    Fetches all course details from CAB in parallel
    """
    print(f"[INFO] Getting details for {len(courses)} courses in parallel...")
    get_details_payload = lambda r: {
        "group": f"code:{r['code']}",
        "key": f"crn:{r['crn']}",
        "srcdb": r["srcdb"],
        "matched": f"crn:{r['crn']}",
    }
    rs = (grequests.post(CAB_DETAIL_SEARCH_URL, json=get_details_payload(r)) for r in courses)
    raw_parallel = grequests.map(rs)
    
    # construct a dictionary of course details by course code
    details = {}
    for response in raw_parallel:
        try:
            response.raise_for_status()
        except requests.exceptions.HTTPError as e:
            print(f"[ERROR] Error fetching course details: {e}")
            continue
        
        response_json = response.json()
        details[response_json["code"]] = response_json
    
    return details


def clean_description(description: str) -> str:
    """
    Cleans the description of each course by removing HTML tags
    """
    return BeautifulSoup(description, 'html.parser').get_text()

