type Role = "engineer" | "system" | "user";
type Message = {
  role: Role;
  contenct: string;
};
type Prompt = Message[];
