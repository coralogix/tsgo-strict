import type { Type1935 } from '../batch-09/file-1935.js';
export interface Type1936 {
  id: 1936;
  name: 'File1936';
  next: Type1935;
}

export function make1936(): Type1936 {
  return { id: 1936, name: 'File1936', next: null as unknown as Type1935 };
}
