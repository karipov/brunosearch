import requests
from bs4 import BeautifulSoup
import html
import re
import json


CAB = "http://cab.brown.edu"


def fetch_and_parse_subjects() -> dict:
    """
    Fetches and parses the list of subjects from CAB
    """

    print(f"[INFO] Fetching and parsing subjects...")
    try:
        # get raw HTML content
        response = requests.get(CAB)
        response.raise_for_status()
    except requests.exceptions.RequestException as e:
        print(f"[ERROR] Error fetching the webpage: {e}")
        return {}

    # parse the HTML content
    soup = BeautifulSoup(response.text, 'html.parser')
    
    # find the <select> tag with id="crit-subject"
    select_tag = soup.find('select', {'id': 'crit-subject'})
    if not select_tag:
        print("[ERROR] No <select> tag found with id='crit-subject'.")
        return {}
    
    # find all <option> tags within the <select> tag
    option_tags = select_tag.find_all('option')
    
    # regular expression to extract value and text
    pattern = r'<option value="(\w+)">([^<]+)\s+\(\w+\)</option>'
    subject_mapping = {}

    for option in option_tags:
        match = re.match(pattern, str(option))
        if match:
            designation = match.group(1)
            name = html.unescape(match.group(2).strip())  # decode weird HTML entities
            subject_mapping[designation] = name
    
    if not subject_mapping:
        print("No subjects found.")
    
    return subject_mapping


if __name__ == "__main__":
    def save_to_json(data: dict, file_path: str):
        with open(file_path, 'w', encoding='utf-8') as file:
            json.dump(data, file, indent=4)
        print(f"Data saved to {file_path}")

    file_path = "data/subjects.json"
    
    print(f"[INFO] Fetching and parsing subjects...")
    subjects = fetch_and_parse_subjects()
    
    # save the results
    if subjects:
        print(f"[INFO] Found {len(subjects)} subjects.")
        print(f"[INFO] Example subjects: {list(subjects.items())[:2]}")
        print(f"[INFO] Saving subjects to {file_path}")
        save_to_json(subjects, file_path)
    print("[INFO] Done!")