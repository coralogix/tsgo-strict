import type { Type3090 } from '../batch-15/file-3090.js';
export interface Type3091 {
  id: 3091;
  name: 'File3091';
  next: Type3090;
}

export function make3091(): Type3091 {
  return { id: 3091, name: 'File3091', next: null as unknown as Type3090 };
}
