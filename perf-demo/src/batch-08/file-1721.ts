import type { Type1720 } from '../batch-08/file-1720.js';
export interface Type1721 {
  id: 1721;
  name: 'File1721';
  next: Type1720;
}

export function make1721(): Type1721 {
  return { id: 1721, name: 'File1721', next: null as unknown as Type1720 };
}
