from ctypes import pythonapi
from kybra import query, update, void


from services.network_manager import NetworkManager








# This is a global variable that is stored on the heap
message: str = ''
# Query calls complete quickly because they do not go through consensus

@query
def get_message() -> str:
    return message

# Update calls take a few seconds to complete
# This is because they persist state changes and go through consensus
@update
def set_message(new_message: str) -> void:
    global message
    message = new_message # This change will be persisted

@query
def get_topic() -> str:
    return message

@update
def set_prompt(faculty: str, year_of_study: int, program_of_study: str, domain: str) -> void:

    data = {}
    data["faculty"] = faculty
    data["year_of_study"] = year_of_study
    data["program_of_study"] = program_of_study
    data["domain"] = domain
    
    global message
    #message = OpenAiManager(faculty=faculty, year_of_study=year_of_study, program_of_study=program_of_study, domain=domain)

    NetworkManager()
    return 