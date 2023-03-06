import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import styled from "styled-components";
import { listen, UnlistenFn, emit } from "@tauri-apps/api/event";

const AppContainer = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;
`;

const Button = styled.button`
  margin-top: 50px;
  width: 100px;
  height: 50px;
`;

const Input = styled.input``;
const FlexBox = styled.div`
  display: flex;
  margin-top: 50px;
  > ${Input} {
    margin-left: 20px;
  }
`;

const SubmitButton = styled.button`
  margin-left: 10px;
`;

const write_answer = async (text: string) => {
    await invoke("write_answer", { text });
}


function App() {
  const { isPlayingRef,audioRef, uri, play, setBase64Uri } = useAudio();
  const [reading, setReading] = useState("");
  const speak = async (text: string) => {
    const base64 = await getWavBase64String(text);
    if (base64 && !isPlayingRef.current) {
      setReading(text);
      setBase64Uri(base64);
    }
    setTimeout(() => {
      if (audioRef.current?.paused && !isPlayingRef.current) { play();} }
      , 0);
  }
  const onSampleValueSubmit = async (sampleValue: string) => {
    const base64 = await getWavBase64String(sampleValue);
    console.log(base64);
    if (base64) {
      setBase64Uri(base64);
    }
  };

  const handleOnChat = (text: string) => {
    console.log(`chat ${text} ${new Date()}`);
    speak(text);
  }

  const onliveChatIdSubmit = async (liveChatId: string) => {
    const url = `https://www.youtube.com/watch?v=${liveChatId}`;
    await invoke("update_client", { liveUrl: url });
  };
  useEffect(() => {
    let unlisten: Promise<UnlistenFn> = 
      listen("chat", (event) => {
          const payload: unknown = event.payload;
          if (typeof payload === "string") {
            handleOnChat(payload);
          }
      });

    return () => {
      unlisten.then(f => f());
    };
  }, []);



  return (
    <AppContainer>
      <audio controls src={uri ?? ""} ref={audioRef}></audio>
      <SubmitValueBox name="sample text" onSubmit={onSampleValueSubmit} />
      <SubmitValueBox name="liveChatId"  onSubmit={onliveChatIdSubmit} />
      <Button onClick={play}>play</Button>
      <Button onClick={() => emit("stop")}>stop</Button>
      <div>NOW READING...: {reading}</div>
      <div>{`isPlaying: ${isPlayingRef.current}`}</div>
    </AppContainer>
  );
}

type SubmitValueBoxProps = {
  name: string,
  onSubmit: (text: string) => void;
}
const SubmitValueBox = ({name, onSubmit}: SubmitValueBoxProps) => {
  const [text, setText] = useState("");
      return (<FlexBox>
        <div>{name}</div>
        <Input
          value={text}
          onChange={(e) => {
            setText(e.target.value);
          }}
        />
        <SubmitButton onClick={() => onSubmit(text)}>
          submit
        </SubmitButton>
      </FlexBox>);
}

const useAudio = () => {
  const audioRef = useRef<HTMLAudioElement | null>(null);
  const isPlayingRef = useRef<boolean>(false);
  const [uri, setUri] = useState<string | null>(null);
  const [isPlaying, setIsPlaying] = useState(false);
  useEffect(() => {
    isPlayingRef.current = isPlaying;
  }, [isPlaying])
  useEffect(() => {
    const audio = audioRef.current;
    function setIsPlayingFalse() {setIsPlaying(false);}
    if (audio) {
      audio.addEventListener("ended", setIsPlayingFalse);
      return () => audio.removeEventListener("ended", setIsPlayingFalse);
    }
  }, [audioRef.current])

  const play = () => {
    const audio = audioRef.current;
    if (audio && !isPlaying) {
      setIsPlaying(true);
      audio.play();
    } else {
      throw new Error("audioRef is null");
    }
  };

  const setBase64Uri = (base64: string) => {
    // base64 => Data URI形式:
    const datauri = "data:audio/wav;base64," + base64;
    setUri(datauri);
  };

  return { isPlayingRef ,uri, audioRef, play, setBase64Uri };
};

async function getWavBase64String(text: string) {
  console.log(`getWavBase64String for ${text}`);
  try {
    const { base64 } = await invoke("get_wav_base64_encoded_string", {
      text,
    }) as { base64: string | null };
    return base64;
  } catch (e) {
    let msg = "err";
    if (e instanceof Error) {
      msg = e.message;
    } else if (typeof e === "string") {
      msg = e;
    }
    console.error(msg);
    return null;
  }
}

export default App;
