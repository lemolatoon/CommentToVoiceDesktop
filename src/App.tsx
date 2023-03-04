import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import styled from "styled-components";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

const AppContainer = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;
`;

const PlayButton = styled.button`
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


function App() {
  const { audioRef, uri, play, setBase64Uri } = useAudio();
  const [sampleValue, setSampleValue] = useState<string | null>(null);
  const [liveChatId, setLiveChatId] = useState<string | null>(null);
  const speak = async (text: string) => {
    const base64 = await getWavBase64String(text);
    if (base64) {
      setBase64Uri(base64);
    }
    await play();
  }
  const onSubmit = async () => {
    if (!sampleValue) return;
    const base64 = await getWavBase64String(sampleValue);
    if (base64) {
      setBase64Uri(base64);
    }
  };

  const handleOnChat = (text: string) => {
    console.log(`chat ${text} ${new Date()}`);
    speak(text);
  }

  const onliveChatIdSubmit = async () => {
    if (!liveChatId) return;
    const url = `https://www.youtube.com/watch?v=${liveChatId}`;
    await invoke("update_client", { liveUrl: url });
  };
  useEffect(() => {
    let unlisten: UnlistenFn | undefined = undefined;
    (async () => {
      unlisten = await listen("chat", (event) => {
          const payload: unknown = event.payload;
          if (typeof payload === "string") {
            handleOnChat(payload);
          }
      });
    })();

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, []);

  return (
    <AppContainer>
      <audio controls src={uri ?? ""} ref={audioRef}></audio>
      <SubmitValueBox name="sample text" value={sampleValue ?? ""} onSubmit={setSampleValue} />
      <SubmitValueBox name="liveChatId" value={liveChatId ?? ""} onSubmit={setLiveChatId} />
      <PlayButton onClick={play}>play</PlayButton>
    </AppContainer>
  );
}

type SubmitValueBoxProps = {
  name: string,
  value: string,
  onSubmit: (text: string) => void;
}
const SubmitValueBox = ({name, value, onSubmit}: SubmitValueBoxProps) => {
  const [text, setText] = useState(value);
      return (<FlexBox>
        <div>{name}</div>
        <Input
          value={value}
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
  const [uri, setUri] = useState<string | null>(null);
  const play = () => {
    const audio = audioRef.current;
    if (audio) {
      return new Promise(async (resolve) => {
        audio.addEventListener("ended", () => resolve(true), { once: true });
        await audio.play();
      });
    } else {
      throw new Error("audioRef is null");
    }
  };

  const setBase64Uri = (base64: string) => {
    // base64 => Data URI形式:
    const datauri = "data:audio/wav;base64," + base64;
    setUri(datauri);
  };

  return { uri, audioRef, play, setBase64Uri };
};

async function getWavBase64String(text: string) {
  try {
    const { base64 } = await invoke("get_wav_base64_encoded_string", {
      text,
    }) as { base64: string };
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
