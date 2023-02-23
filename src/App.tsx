import { useEffect, useMemo, useRef, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import styled from "styled-components";
import { dialog } from "@tauri-apps/api";
import { LiveChat } from "youtube-chat";

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
  const onSubmit = async () => {
    if (!sampleValue) return;
    const base64 = await getWavBase64String(sampleValue);
    if (base64) {
      setBase64Uri(base64);
    }
  };

  return (
    <AppContainer>
      <audio controls src={uri ?? ""} ref={audioRef}></audio>
      <FlexBox>
        <div>sample text</div>
        <Input value={sampleValue ?? ""} onChange={(e) => {setSampleValue(e.target.value);}} />
        <SubmitButton onClick={onSubmit}>
          submit
        </SubmitButton>
      </FlexBox>
      <FlexBox>
        <div>liveChatId</div>
        <Input value={liveChatId ?? ""} onChange={(e) => {setLiveChatId(e.target.value);}} />
      </FlexBox>
      <PlayButton onClick={play}>play</PlayButton>
    </AppContainer>
  );
}

const useAudio = () => {
  const audioRef = useRef<HTMLAudioElement | null>(null);
  const [uri, setUri] = useState<string | null>(null); 
  const play = () => {
    const audio = audioRef.current;
    if (audio) {
      return new Promise(async (resolve) => {
        audio.addEventListener("ended", () => resolve(true), {once: true});
        await audio.play();
      }
      );
    } else {
      throw new Error("audioRef is null");
    }
  }

  const setBase64Uri = (base64: string) => {
    // base64 => Data URI形式:
    const datauri = "data:audio/wav;base64," + base64;
    setUri(datauri);
  }

  return { uri, audioRef, play, setBase64Uri };
}

async function getWavBase64String(text: string) {
  try {
    const { base64 } = await invoke("get_wav_base64_encoded_string", {text}) as {base64: string};
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
