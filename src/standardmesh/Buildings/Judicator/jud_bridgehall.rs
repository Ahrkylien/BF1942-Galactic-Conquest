subshader "Material #2" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 1.0 1.0 1.0;
	materialSpecular 0.541176 0.541176 0.541176;
	materialSpecularPower 14.5;
	envmap true;
	texture "texture/buildings/judicator/jud_floor";
}

subshader "jud_bridgehall_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 1.0 0.0862745 0.882353;
	texture "texture/buildings/judicator/jud_floornospec";
}

subshader "jud_bridgehall_Material2" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.65098 0.588235 0.596078;
	texture "texture/buildings/judicator/jud_wall01";
}

