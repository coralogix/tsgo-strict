import type { Type3302 } from '../batch-16/file-3302.js';
export interface Type3303 {
  id: 3303;
  name: 'File3303';
  next: Type3302;
}

export function make3303(): Type3303 {
  return { id: 3303, name: 'File3303', next: null as unknown as Type3302 };
}
