import type { Type1023 } from '../batch-05/file-1023.js';
export interface Type1024 {
  id: 1024;
  name: 'File1024';
  next: Type1023;
}

export function make1024(): Type1024 {
  return { id: 1024, name: 'File1024', next: null as unknown as Type1023 };
}
