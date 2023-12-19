import openai
from open_ai_configurations import OpenAiConfigurations


class OpenAiManager:
    def __init__(self, program_of_study, faculty, domain, year_of_study):
        self.program_of_study = program_of_study
        self.faculty = faculty
        self.domain = domain
        self.year_of_study = year_of_study
        self.open_ai_api_key = OpenAiConfigurations.OPEN_AI_API_KEY

    def generate_project_topics(self):
        prompt = f'I am a {self.year_of_study} year University student studying towards my {self.program_of_study} degree from the faculty of {self.faculty}. I have been tasked with comming up with a topic for my {self.year_of_study} year Capstone project. A research topic is the subject of a research project or study – for example, a dissertation or thesis. A research topic typically takes the form of a problem to be solved, or a question to be answered. A good research topic should be specific enough to allow for focused research and analysis. Generate me five detailed research topics I can do my Capstone project on. I want my project to solve a problem in the {self.domain} domain. For each topics include a description of the problem it solves. Format your response as markdown'
        openai.api_key = OpenAiConfigurations.OPEN_AI_API_KEY
        response = openai.Completion.create(
            engine="text-davinci-002",
            prompt=prompt,
            max_tokens=OpenAiConfigurations.OPEN_AI_MAX_TOKENS,
            temperature=OpenAiConfigurations.OPEN_AI_TEMPERATURE)
        return response
