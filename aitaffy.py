import io
import sys

from gradio_client import Client


def test(msg):
    client = Client("https://www.modelscope.cn/api/v1/studio/xzjosh/LittleTaffy-Bert-VITS2/gradio/")
    result = client.predict(
        msg,  # str in 'Text' Textbox component
        "TF",  # str (Option from: ['TF']) in 'Speaker' Dropdown component
        0.2,  # int | float (numeric value between 0 and 1) in 'SDP/DP混合比' Slider component
        0.6,  # int | float (numeric value between 0.1 and 1.5) in '感情调节' Slider component
        0.8,  # int | float (numeric value between 0.1 and 1.4) in '音素长度' Slider component
        1,  # int | float (numeric value between 0.1 and 2) in '生成长度' Slider component
        fn_index=0
    )
    print(str(result)[13:len(str(result)) - 2])


if __name__ == '__main__':
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf8')  # 改变标准输出的默认编码
    if len(sys.argv) < 2:
        exit(0)
    msg = sys.argv[1]
    test(msg)
