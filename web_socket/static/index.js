const wsUrl = 'ws://localhost:3000/ws';
const root = document.querySelector(".root")
const message = document.querySelector(".message")

const conn = new WebSocket(wsUrl);
console.log('连接中！');

conn.addEventListener('open', function () {
    console.log('已连接')
})

conn.addEventListener('message', function (e) {
    console.log('对应消息数据', e)
    message.appendChild(`${e.data} \n`)
})

conn.addEventListener('close', function () {
    console.log('消息通道已关闭');
    conn = null;
})

const sendInput = document.createElement('input');
const sendDom = document.createElement('div');
const textNode = document.createTextNode('发送按钮')
sendDom.appendChild(textNode)
let value = null;

sendInput.addEventListener('change', (e) => {
    value = e.target.value
})

sendDom.addEventListener('click', () => {
    conn.send(value)
})



