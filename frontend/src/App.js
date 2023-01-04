import logo from './logo.svg';
import useWebSocket, { ReadyState } from 'react-use-websocket';
import { useState, useEffect } from 'react';
import { string2Arraybuffer, arraybuffer2String } from './util'

function App() {
  const [socketUrl, setSocketUrl] = useState('ws://localhost:9700/ws');
  const [messageList, setMessageList] = useState([]);

  const { sendMessage, lastMessage, readyState, getWebSocket } = useWebSocket(
    socketUrl,
    { share: true }
  );
  useEffect(() => {
    if (readyState === ReadyState.OPEN) {
      //Change binaryType property of WebSocket
      getWebSocket().binaryType = 'arraybuffer';
    }
  }, [readyState]);

  const [inputValue, setInputValue] = useState('');

  const connectionStatus = {
    [ReadyState.CONNECTING]: 'Connecting',
    [ReadyState.OPEN]: 'Open',
    [ReadyState.CLOSING]: 'Closing',
    [ReadyState.CLOSED]: 'Closed',
    [ReadyState.UNINSTANTIATED]: 'Uninstantiated',
  }[readyState];

  useEffect(() => {
    if (lastMessage !== null) {
      console.log(lastMessage);
      setMessageList((prev) => prev.concat(lastMessage));
    }
  }, [lastMessage, setMessageList]);

  const send = () => {
    console.log(string2Arraybuffer(inputValue))
    // getWebSocket().send(string2Arraybuffer(inputValue));
    sendMessage(string2Arraybuffer(inputValue));
    setInputValue('');
  }

  return (
    <div>
      <p>websocket state: {connectionStatus}</p>
      <ul>
        {messageList.map((msg) => <li key={msg.timeStamp}>{arraybuffer2String(msg.data)}</li>)}
      </ul>
      <input value={inputValue} onChange={(e) => setInputValue(e.target.value)} />
      <button onClick={send}>发送</button>
    </div>
  );
}

export default App;
