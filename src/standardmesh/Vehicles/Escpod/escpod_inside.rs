subshader "escpod_inside_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 1.0 1.0 1.0;
	texture "texture/vehicles/escpod/podinterior";
}

subshader "escpod_inside_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 0.588 0.588 0.588;
	materialSpecular 1.0 1.0 1.0;
	materialSpecularPower 1.0;
	transparent true;
	envmap true;
	depthwrite true;
	texture "texture/vehicles/escpod/window";
}

