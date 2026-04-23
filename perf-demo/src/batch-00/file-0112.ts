import type { Type111 } from '../batch-00/file-0111.js';
export interface Type112 {
  id: 112;
  name: 'File112';
  next: Type111;
}

export function make112(): Type112 {
  return { id: 112, name: 'File112', next: null as unknown as Type111 };
}
