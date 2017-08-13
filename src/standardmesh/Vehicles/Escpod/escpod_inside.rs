subshader "escpod_inside_Material0" "StandardMesh/Default"
{
	lighting true;
	envmap true;
	materialDiffuse 1.0 1.0 1.0;
	texture "texture/vehicles/escpod/podinterior";
}

subshader "escpod_inside_Material1" "StandardMesh/Default"
{
	lighting true;
	envmap true;
	materialDiffuse 1.0 1.0 1.0;
	transparent true;
	depthwrite true;
	texture "texture/vehicles/escpod/window";
}

