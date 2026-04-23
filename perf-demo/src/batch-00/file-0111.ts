import type { Type110 } from '../batch-00/file-0110.js';
export interface Type111 {
  id: 111;
  name: 'File111';
  next: Type110;
}

export function make111(): Type111 {
  return { id: 111, name: 'File111', next: null as unknown as Type110 };
}
