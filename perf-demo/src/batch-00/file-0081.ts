import type { Type80 } from '../batch-00/file-0080.js';
export interface Type81 {
  id: 81;
  name: 'File81';
  next: Type80;
}

export function make81(): Type81 {
  return { id: 81, name: 'File81', next: null as unknown as Type80 };
}
