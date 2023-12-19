from ctypes import pythonapi
from kybra import query, update, void
from open_ai.open_ai_manager import OpenAiManager


from services.network_manager import NetworkManager,OpenAiConfigurations

prompt_response = ""



@query
def get_prompt_response() -> str:
    return prompt_response


@update
def set_prompt(faculty: str, year_of_study: int, program_of_study: str, domain: str) -> void:

    data = {}
    data["faculty"] = faculty
    data["year_of_study"] = year_of_study
    data["program_of_study"] = program_of_study
    data["domain"] = domain

    open_ai = OpenAiManager(
        program_of_study=program_of_study,
        faculty=faculty,
        domain=domain,
        year_of_study=year_of_study)
    
    global prompt_response
    promot_response = open_ai.generate_project_topics()
    
    return 