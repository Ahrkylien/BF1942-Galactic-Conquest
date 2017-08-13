subshader "jud_fakedoor04_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
      materialSpecular 0.8 0.8 0.8;
	materialSpecularPower 4.0;
	envmap true;
	materialDiffuse 0.980392 0.592157 0.592157;
	texture "texture/buildings/judicator/jud_floor";
}

subshader "jud_fakedoor04_Material1" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 0.588235 0.588235 0.588235;
	envmap true;
	lightingSpecular true;
	texture "texture/buildings/judicator/jud_bulkhead";
}

subshader "jud_fakedoor04_Material2" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 0.588235 0.588235 0.588235;
	envmap true;
	lightingSpecular true;
	texture "texture/buildings/judicator/jud_floornospec";
}

subshader "jud_fakedoor04_Material3" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 0.588235 0.588235 0.588235;
	envmap true;
	lightingSpecular true;
	texture "texture/buildings/judicator/jud_door01";
}

