from gtts import gTTS

# SEARCH FOR BETTER MODEL, THIS ISN'T GOOD

def use_tts(message_to_transcribe: str):
    tts = gTTS(message_to_transcribe)
    path = "./../data/output.mp3"
    tts.save(path)

    output_bytes = 0x0

    with open("myfile", "rb") as f:
    while (byte := f.read(1)):
        output_bytes = byte
