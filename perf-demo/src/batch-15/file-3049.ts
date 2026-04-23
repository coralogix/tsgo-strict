import type { Type3048 } from '../batch-15/file-3048.js';
export interface Type3049 {
  id: 3049;
  name: 'File3049';
  next: Type3048;
}

export function make3049(): Type3049 {
  return { id: 3049, name: 'File3049', next: null as unknown as Type3048 };
}
