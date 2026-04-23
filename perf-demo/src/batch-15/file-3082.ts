import type { Type3081 } from '../batch-15/file-3081.js';
export interface Type3082 {
  id: 3082;
  name: 'File3082';
  next: Type3081;
}

export function make3082(): Type3082 {
  return { id: 3082, name: 'File3082', next: null as unknown as Type3081 };
}
