subshader "Material #2" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 1.0 1.0 1.0;
	envmap true;
	texture "texture/buildings/judicator/jud_floor";
}

subshader "jud_bridgehall_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588235 0.588235 0.588235;
	envmap true;
	texture "texture/buildings/judicator/jud_floornospec";
}

subshader "jud_bridgehall_Material2" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588235 0.588235 0.588235;
	envmap true;
	texture "texture/buildings/judicator/jud_wall01";
}

