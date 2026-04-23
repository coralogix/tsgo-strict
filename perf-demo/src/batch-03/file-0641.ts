import type { Type640 } from '../batch-03/file-0640.js';
export interface Type641 {
  id: 641;
  name: 'File641';
  next: Type640;
}

export function make641(): Type641 {
  return { id: 641, name: 'File641', next: null as unknown as Type640 };
}
