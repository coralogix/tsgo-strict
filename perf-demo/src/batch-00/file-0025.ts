import type { Type24 } from '../batch-00/file-0024.js';
export interface Type25 {
  id: 25;
  name: 'File25';
  next: Type24;
}

export function make25(): Type25 {
  return { id: 25, name: 'File25', next: null as unknown as Type24 };
}
