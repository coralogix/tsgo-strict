import type { Type401 } from '../batch-02/file-0401.js';
export interface Type402 {
  id: 402;
  name: 'File402';
  next: Type401;
}

export function make402(): Type402 {
  return { id: 402, name: 'File402', next: null as unknown as Type401 };
}
