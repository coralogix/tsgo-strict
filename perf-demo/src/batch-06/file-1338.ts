import type { Type1337 } from '../batch-06/file-1337.js';
export interface Type1338 {
  id: 1338;
  name: 'File1338';
  next: Type1337;
}

export function make1338(): Type1338 {
  return { id: 1338, name: 'File1338', next: null as unknown as Type1337 };
}
