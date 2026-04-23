import type { Type1934 } from '../batch-09/file-1934.js';
export interface Type1935 {
  id: 1935;
  name: 'File1935';
  next: Type1934;
}

export function make1935(): Type1935 {
  return { id: 1935, name: 'File1935', next: null as unknown as Type1934 };
}
