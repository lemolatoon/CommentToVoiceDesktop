import { useState } from "react";
import styled from "styled-components";

export type Reply = {
  role: "user" | "ai";
  content: string;
};
type RepliesProps = {
  replies: Reply[];
};
const Role = styled.div`
  font-size: 0.5rem;
`;
const Content = styled.div`
  font-size: 1.5rem;
`;
const ReplyGrid = styled.div`
  display: grid;
  grid-template-rows: 1fr 3fr;
  > ${Role} {
    grid-row-start: 0;
    grid-row-end: 1;
  }
  > ${Content} {
    grid-row-start: 1;
    grid-row-end: 2;
  }
`;

const RepliesWrapper = styled.div`
  overflow-y: scroll;
`;
export const Replies = ({ replies }: RepliesProps) => {
  return (
    <RepliesWrapper>
      {replies.map((reply) => {
        return (
          <ReplyGrid>
            <Role>{reply.role}</Role>
            <Content>{reply.content}</Content>
          </ReplyGrid>
        );
      })}
    </RepliesWrapper>
  );
};

export const useReplies = () => {
  const [replies, setReplies] = useState<Reply[]>([]);

  const push_reply = (reply: Reply) => {
    setReplies((replies) => [...replies, reply]);
  };

  return { replies, push_reply };
};
