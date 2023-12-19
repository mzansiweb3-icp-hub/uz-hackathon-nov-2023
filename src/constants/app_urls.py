class AppUrls:
    base_url = "https://api.openai.com/v1/"
    chat = None

    def __init__(self):
        self.chat = Chat()



class Chat:
    def __init__(self) -> None:
        self.chat = "chat/completions"
        