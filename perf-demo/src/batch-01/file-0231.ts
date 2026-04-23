import type { Type230 } from '../batch-01/file-0230.js';
export interface Type231 {
  id: 231;
  name: 'File231';
  next: Type230;
}

export function make231(): Type231 {
  return { id: 231, name: 'File231', next: null as unknown as Type230 };
}
