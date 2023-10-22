import io
import sys

from gradio_client import Client

def test(msg):
    client = Client("https://skytnt-moe-tts.hf.space/")
    result = client.predict(
				msg,	# str in 'Text (150 words limitation)' Textbox component
				"綾地寧々",	# str (Option from: ['綾地寧々', '因幡めぐる', '朝武芳乃', '常陸茉子', 'ムラサメ', '鞍馬小春', '在原七海']) in 'Speaker' Dropdown component
				0.5,	# int | float (numeric value between 0.5 and 2) in 'Speed' Slider component
				True,	# bool in 'Symbol input' Checkbox component
				fn_index=1
    )
    print(str(result)[13:len(str(result)) - 2])
    
    
if __name__ == '__main__':
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf8')  # 改变标准输出的默认编码
    if len(sys.argv) < 2:
        exit(0)
    msg = sys.argv[1]
    test(msg)
