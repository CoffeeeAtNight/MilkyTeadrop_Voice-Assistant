from gtts import gTTS

# SEARCH FOR BETTER MODEL, THIS ISN'T GOOD

def use_tts(message_to_transcribe: str):
    tts = gTTS(message_to_transcribe)
    tts.save('./../data/output.mp3')