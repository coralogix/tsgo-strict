import type { Type3232 } from '../batch-16/file-3232.js';
export interface Type3233 {
  id: 3233;
  name: 'File3233';
  next: Type3232;
}

export function make3233(): Type3233 {
  return { id: 3233, name: 'File3233', next: null as unknown as Type3232 };
}
