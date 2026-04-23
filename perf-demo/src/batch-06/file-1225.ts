import type { Type1224 } from '../batch-06/file-1224.js';
export interface Type1225 {
  id: 1225;
  name: 'File1225';
  next: Type1224;
}

export function make1225(): Type1225 {
  return { id: 1225, name: 'File1225', next: null as unknown as Type1224 };
}
