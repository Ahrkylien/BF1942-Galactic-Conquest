subshader "7 - Default" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 1.0 1.0 1.0;
	transparent true;
        depthWrite false;
	texture "texture/buildings/superecho/gmapglass";
}

subshader "escpod_inside_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588 0.588 0.588;
	texture "texture/buildings/superecho/part_2_c_roof";
}

