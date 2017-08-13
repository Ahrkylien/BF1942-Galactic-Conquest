subshader "mc_pipe_curve_Material0" "StandardMesh/Default"
{
	lighting true;
	materialSpecular 0.117647 0.117647 0.117647;
	materialSpecularPower 12.5;
        lightingSpecular true;
	envmap true;
	transparent true;
        twosided true;
	materialDiffuse 0.588 0.588 0.588;
	texture "texture/buildings/moncal/mc_waterpipe2";
}

subshader "mc_pipe_curve_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588 0.588 0.588;
	twosided true;
	texture "texture/buildings/moncal/mc_metal1";
}

