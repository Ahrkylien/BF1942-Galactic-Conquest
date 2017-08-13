subshader "imp_escapepod_inside_Material0" "StandardMesh/Default"
{
	lighting true;
	envmap true;
	materialDiffuse 1.0 1.0 1.0;
	texture "texture/vehicles/impescpod/imp_escapepod_inside";
}

subshader "imp_escapepod_inside_Material1" "StandardMesh/Default"
{
	lighting true;
	envmap true;
	materialDiffuse 1.0 1.0 1.0;
	transparent true;
      	depthWrite true;
	texture "texture/vehicles/lancer/transparent";
}

