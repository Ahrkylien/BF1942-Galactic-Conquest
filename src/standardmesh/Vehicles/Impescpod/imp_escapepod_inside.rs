subshader "imp_escapepod_inside_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588235 0.588235 0.588235;
	texture "texture/vehicles/impescpod/imp_escapepod_inside";
}

subshader "imp_escapepod_inside_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588 0.588 0.588;
	transparent true;
      depthWrite true;
	texture "texture/vehicles/lancer/transparent";
}

