import { useEffect, useRef, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import styled from "styled-components";
import { dialog } from "@tauri-apps/api";

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

const FlexBox = styled.div`
  display: flex;
  margin-top: 50px;
`;

const SubmitButton = styled.button`
  margin-left: 10px;
`;

const Input = styled.input``;

function App() {
  const { audioRef, sourceRef, play, setByteArray } = useAudio();
  const [sampleValue, setSampleValue] = useState<string | null>(null);
  const onSubmit = async () => {
    if (!sampleValue) return;
    const bytes = await getWavByteArray(sampleValue);
    console.log(`bytes: ${bytes}`);
    if (bytes) {
      setByteArray(bytes);
    }
    console.log(`${sampleValue}, submitted`);
  };

  return (
    <AppContainer>
      <audio ref={audioRef} controls>
        <source ref={sourceRef} type="audio/wav" />
      </audio>
      <FlexBox>
        <Input value={sampleValue ?? ""} onChange={(e) => setSampleValue(e.target.value)} />
        <SubmitButton onClick={onSubmit}>
          submit
        </SubmitButton>
      </FlexBox>
      <PlayButton onClick={play}>play</PlayButton>
    </AppContainer>
  );
}

const useAudio = () => {
  const audioRef = useRef<HTMLAudioElement>(null);
  const sourceRef = useRef<HTMLSourceElement>(null);
  const play = () => {
    const audioElm =audioRef.current;
    if (audioElm) {
      return new Promise(async (resolve) => {
        audioElm.addEventListener("ended", () => resolve(true), {once: true});
        console.log(`play src: ${sourceRef.current?.src ?? "null"}`);
        console.log("play");
        audioElm.load();
        await audioElm.play();
      }
      );
    } else {
      throw new Error("audioRef is null");
    }
  }

  const setByteArray = (byteString: Uint8Array) => {
    function getByteArray() {
      const data = [82, 73, 70, 70, 222, 37, 0, 0, 87, 65, 86, 69, 102, 109, 116, 32, 16, 0, 0, 0, 1, 0, 1, 0, 68, 172, 0, 0, 136, 88, 1, 0, 2, 0, 16, 0, 100, 97, 116, 97, 186, 37, 0, 0, 0, 0, 255, 12, 2, 27, 254, 40, 2, 55, 254, 68, 1, 83, 0, 83, 0, 69, 0, 55, 255, 40, 2, 27, 253, 12, 3, 255, 254, 240, 0, 227, 1, 213, 255, 198, 1, 185, 255, 170, 1, 175, 255, 188, 1, 203, 255, 216, 1, 231, 255, 244, 2, 3, 254, 16];

      // Convert byteArray into Uint8Array.
      return new Uint8Array(data);
    }
    const blob = new Blob([byteString], {type: "audio/wav"});
    // const blob = new Blob([getByteArray()], {type: "audio/wav"});
    const url = URL.createObjectURL(blob);
    console.log(`url: ${url}`);
    if (sourceRef.current) {
      sourceRef.current.src = url;
    }
  }

  return { audioRef, sourceRef, play, setByteArray };
}

async function getWavByteArray(text: string) {
  try {
    const { bytes } = await invoke("get_wav_byte_string", {text}) as {bytes: [number]};
    return new Uint8Array(bytes);
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
