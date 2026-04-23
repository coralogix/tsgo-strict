import type { Type901 } from '../batch-04/file-0901.js';
export interface Type902 {
  id: 902;
  name: 'File902';
  next: Type901;
}

export function make902(): Type902 {
  return { id: 902, name: 'File902', next: null as unknown as Type901 };
}
