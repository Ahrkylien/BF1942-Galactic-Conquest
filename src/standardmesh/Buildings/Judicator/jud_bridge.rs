subshader "Material #2" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 1.0 1.0 1.0;
	materialSpecular 1.0 1.0 1.0;
	materialSpecularPower 14;
	envmap true;
	texture "texture/buildings/judicator/jud_floor";
}

subshader "jud_bridge_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.113726 0.0 0.972549;
	texture "texture/buildings/judicator/jud_wall02";
}

subshader "jud_bridge_Material2" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 1.0 0.0862745 0.882353;
	texture "texture/buildings/judicator/jud_floornospec";
}

subshader "jud_bridge_Material3" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.984314 0.164706 0.164706;
	texture "texture/buildings/judicator/jud_frontpanel";
}

subshader "jud_bridge_Material4" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.72549 0.6 0.607843;
	texture "texture/buildings/judicator/jud_pipes";
}

subshader "jud_bridge_Material5" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.65098 0.588235 0.596078;
	texture "texture/buildings/judicator/jud_wall01";
}

subshader "Material #11" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 1.0 1.0 1.0;
	materialSpecular 1.0 1.0 1.0;
	materialSpecularPower 14.0;
	transparent true;
	texture "texture/buildings/judicator/jud_glass";
}

