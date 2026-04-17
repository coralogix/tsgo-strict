import type { Type8 } from '../batch-00/file-0008.js';
export interface Type9 {
  id: 9;
  name: 'File9';
  next: Type8;
}

export function make9(): Type9 {
  return { id: 9, name: 'File9', next: null as unknown as Type8 };
}
