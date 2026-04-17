import type { Type20 } from '../batch-00/file-0020.js';
export interface Type21 {
  id: 21;
  name: 'File21';
  next: Type20;
}

export function make21(): Type21 {
  return { id: 21, name: 'File21', next: null as unknown as Type20 };
}
