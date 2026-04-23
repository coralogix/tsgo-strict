import type { Type60 } from '../batch-00/file-0060.js';
export interface Type61 {
  id: 61;
  name: 'File61';
  next: Type60;
}

export function make61(): Type61 {
  return { id: 61, name: 'File61', next: null as unknown as Type60 };
}
