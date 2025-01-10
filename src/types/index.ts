export interface BotState {
  lastCheckedMentionId: string | null;
  conversations: Map<string, Conversation>;
  userInteractions: Map<string, UserInteraction>;
}

export interface Conversation {
  mentionId: string;
  authorId: string;
  text: string;
  timestamp: Date;
}

export interface UserInteraction {
  userId: string;
  interactionCount: number;
  lastInteraction: Date;
}

export interface TwitterMention {
  id: string;
  authorId: string;
  text: string;
}
