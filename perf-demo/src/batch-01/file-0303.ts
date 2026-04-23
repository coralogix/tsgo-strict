import type { Type302 } from '../batch-01/file-0302.js';
export interface Type303 {
  id: 303;
  name: 'File303';
  next: Type302;
}

export function make303(): Type303 {
  return { id: 303, name: 'File303', next: null as unknown as Type302 };
}
