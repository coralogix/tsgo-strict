import type { Type3720 } from '../batch-18/file-3720.js';
export interface Type3721 {
  id: 3721;
  name: 'File3721';
  next: Type3720;
}

export function make3721(): Type3721 {
  return { id: 3721, name: 'File3721', next: null as unknown as Type3720 };
}
