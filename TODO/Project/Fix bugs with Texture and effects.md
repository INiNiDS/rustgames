# 🐛 Bug Tracking: Textures & Effects

## 1. Texture Rendering Fixes
- [ ] Audit `wgpu` bind group layouts to ensure texture and sampler bindings match the shader expectations.
- [ ] Check texture coordinate (UV) mapping. Ensure coordinates are not flipped (common issue between different graphics APIs).
- [ ] Verify texture format compatibility (e.g., sRGB vs linear) during the `create_texture` phase.
- [ ] Debug depth testing: ensure the depth buffer is properly cleared and depth writes are enabled for opaque geometry.

## 2. Check why effects in examples dont work