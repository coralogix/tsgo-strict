import type { Type5 } from '../batch-00/file-0005.js';
export interface Type6 {
  id: 6;
  name: 'File6';
  next: Type5;
}

export function make6(): Type6 {
  return { id: 6, name: 'File6', next: null as unknown as Type5 };
}
