import type { Type1982 } from '../batch-09/file-1982.js';
export interface Type1983 {
  id: 1983;
  name: 'File1983';
  next: Type1982;
}

export function make1983(): Type1983 {
  return { id: 1983, name: 'File1983', next: null as unknown as Type1982 };
}
