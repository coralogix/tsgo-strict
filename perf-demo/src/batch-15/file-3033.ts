import type { Type3032 } from '../batch-15/file-3032.js';
export interface Type3033 {
  id: 3033;
  name: 'File3033';
  next: Type3032;
}

export function make3033(): Type3033 {
  return { id: 3033, name: 'File3033', next: null as unknown as Type3032 };
}
